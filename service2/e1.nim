import std/[times, os, strutils]
import posix, strformat
import asynchttpserver, asyncdispatch
import httpclient, httpcore

var startTime: DateTime
proc initStartTime() =
    startTime = now()

proc getUptime(): float =
    let elapsed = now() - startTime
    let uptime = float(elapsed.inSeconds()) / 3600.0
    return uptime


proc getFreeDiskMB(path: string = "/") : float =
    var stat: Statvfs
    if statvfs(path, stat) == 0:
        result = stat.f_bavail.float * stat.f_frsize.float
        result = result / (1024.0 * 1024.0)  # Convert bytes to MB 
    else:
        result = -1.0


proc getTimestamp(): string =
    result = now().format("yyyy-MM-dd'T'HH:mm:ss'Z'")


proc postToStorage(record: string) {.async.} =
    let client = newAsyncHttpClient()
    var headers = newHttpHeaders()
    headers.add("Content-Type", "text/plain")
    

    try:
        let response = await client.request("http://storage:5000/log",
            HttpPost,
            body = record,
            headers = headers)
    except HttpRequestError as e:
        echo "Failed to post to storage: ", e.msg


proc logToVStorage(record: string) =
    let path = "/vstorage/log.txt"
    var f = open(path, fmAppend)
    f.writeLine(record)
    f.close()

proc cb(req: Request) {.async.} =
  if req.url.path == "/status":
    let uptime = getUptime()
    let freeDisk = getFreeDiskMB("/")
    let timestamp = getTimestamp() 
    let record = fmt"{timestamp}: uptime {uptime:.5f} hours, free disk in root: {freeDisk:.2f} MBytes"

    await postToStorage(record)

    logToVStorage(record)

    var headers = newHttpHeaders()
    headers["Content-Type"] = "text/plain"
    await req.respond(Http200, record, headers)
  else:
    await req.respond(Http404, "Not found")

when isMainModule:
  initStartTime()

  var server = newAsyncHttpServer()
  echo "Service2 device listening on port 8080"
  waitFor server.serve(Port(8080), cb)