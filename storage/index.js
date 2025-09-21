const express = require('express');
const bodyParser = require('body-parser');
const fs = require('fs');
const path = require('path');


const app = express();
const PORT = 5000;

app.use(bodyParser.text({ type: "text/plain"}));

const LOG_FILE = "/data/log.txt";

// Ensure log file exists at startup
if (!fs.existsSync(LOG_FILE)) {
    fs.writeFileSync(LOG_FILE, "");
}

app.post("/log", bodyParser.text({ type: "text/plain" }), (req, res) => {
    console.log(req.body);
    const record = req.body + '\n';
    fs.appendFile(LOG_FILE, record, (err) => {
        if (err) {
            console.error(err);
            return res.status(500).send("Error while writing to log file.");
        }
        res.sendStatus(200);
    })
});

app.get("/log", (rew, res) => {
    fs.readFile(LOG_FILE, 'utf8', (err, data) => {
        if (err) {
            if (err.code === 'ENONET') {
                return res.send("");
            }
            console.error(err);
            return res.status(500).send("Error while reading log file.");
        }
        res.set("Content-Type", "text/plain");
        res.send(data);
    }); 
});


app.listen(PORT, () => {
    console.log("Storage device listening on port " + PORT);
});