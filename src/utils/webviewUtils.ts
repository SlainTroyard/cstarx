import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';

import { execSync } from 'child_process';
import { env } from 'process';
import { loadavg } from 'os';

let intervalId: NodeJS.Timeout;
let count = 0;
// 生成一个随机的 nonce 值
function getNonce() {
    let text = '';
    const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    for (let i = 0; i < 32; i++) {
        text += possible.charAt(Math.floor(Math.random() * possible.length));
    }
    return text;
}




// 保存输入内容到暫存目录 linux: /tmp/cstas/input
function saveInput(input: string) {
	// const os = require('node:os'); 
	// console.log("Go into save function.");
	// console.log('os.tmpdir()=', os.tmpdir()); 
	var env = Object.assign({}, process.env);
	const pluginRoot = path.resolve(__dirname, '..', '..');
	const inputdir = path.join(pluginRoot, 'trans/input');
	if(inputdir) {
		const filePath = path.join(inputdir, 'input_file');
		fs.writeFileSync(filePath, input, { encoding: 'utf8' });
		vscode.window.showInformationMessage('Input saved successfully.');
	} else {
		// vscode.window.showErrorMessage('os.tmpdir() return undefined.');
	}
}

function callpython(webview :vscode.Webview) {
	console.log("call python proccess.");	
	const pluginRoot = path.resolve(__dirname, '..', '..');
	const transRoot = path.join(pluginRoot, 'trans');
	console.log(env);
	try {
		execSync('python run.py', {
			cwd: transRoot,
			env: env
		});
		// clearInterval(intervalId);
		sendstatus("Translate process success.", webview);
	} catch (err: any) {
		// clearInterval(intervalId);
		sendstatus(err.message, webview);
	}
	
	// execSync('python zp_pro.py');
}

function printOutput(webview :vscode.Webview) {
	console.log("Go into print function.");
	const pluginRoot = path.resolve(__dirname, '..', '..');
	const transRoot = path.join(pluginRoot, 'trans');
	const successfilePath = path.join(transRoot, 'success');
	
	if(fs.existsSync(successfilePath)) {
		fs.readdir(successfilePath, (err, files) => {
			if (err) {
				console.error('Error reading directory:', err);
				return;
			}
			if(files.length > 1){
					return;
			}
			const successFile = path.join(successfilePath, files[0]);
			const content = fs.readFileSync(successFile, { encoding: 'utf8' });
			  webview.postMessage( {
							command: 'received_content',
							content: content
						}
				);

		});
	} else {
		// vscode.window.showErrorMessage('output.rs not exists.');
        console.log('successfilePath not exists.',successfilePath);
	}
	
}
  
function saveAPI(input: string) {
	const pluginRoot = path.resolve(__dirname, '..', '..');
	const transRoot = path.join(pluginRoot, 'trans');
	const apifilePath = path.join(transRoot, 'api_key');
	fs.writeFileSync(apifilePath, input, { encoding: 'utf8' });
	console.log('write api key success.');
}

function sendstatus(status:string, webview :vscode.Webview) {
	webview.postMessage( {
		command: 'received_status',
		content: status
	});
}



function sendtranslateStatus(webview :vscode.Webview) {
	count = 0;
	clearInterval(intervalId);
	console.log("runing timer.");
	intervalId = setInterval(() => {
		const status = "Running translate process time : " + count;
		sendstatus(status, webview);
        count++;
    }, 1000);
	console.log("setInterval done.");
}

export class SidebarProvider implements vscode.WebviewViewProvider {
	constructor(protected context: vscode.ExtensionContext) {}
    _view?: vscode.WebviewView;
    _doc?: vscode.TextDocument;
  
	public resolveWebviewView(webviewView: vscode.WebviewView) {
      this._view = webviewView;
      const nonce = getNonce();
	  webviewView.webview.options = {
		enableScripts: true,
		localResourceRoots: [this.context.extensionUri],
	  };
  
	  webviewView.webview.html = 
      `
	  <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Input Panel</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }
        h1 {
            text-align: center;
        }
        #inputPanel, #outputPanel {
            width: 80%;
            margin: 0 auto;
            padding: 20px;
            border: 1px solid #ccc;
            border-radius: 8px;
            margin-bottom: 20px;
        }
        textarea, input{
            width: 100%;
            padding: 10px;
            margin-bottom: 10px;
            border: 1px solid #ccc;
            border-radius: 6px;
            box-sizing: border-box;
			color: white; /* 文字颜色白色 */
			background-color: black;
        }
        button {
            padding: 10px 20px;
            margin-right: 10px;
            background-color: #007bff;
            color: #fff;
            border: none;
            border-radius: 6px;
            cursor: pointer;
            transition: background-color 0.3s ease;
        }
        button:hover {
            background-color: #0056b3;
        }
    </style>
</head>
<body>
<input type="text" id="keyField" placeholder="Enter your zuipu API key..."><br>
    <h1>Input Panel</h1>
    <div id="inputPanel">
        <textarea id="inputField" rows="20" cols="50"></textarea><br>
        <button id="Translate_button">Translate</button>
        <button id="API_key_button">ReadAPIkey</button>
    </div>
    <h1>Output Panel</h1>
    <div id="outputPanel">
        <textarea readonly id="outputField" rows="20" cols="50" readonly></textarea><br>
    </div>
	<div id="status" color: white>aa</div>
    <script nonce="${nonce}">
        document.getElementById("Translate_button").addEventListener("click", translateInput);
        document.getElementById("API_key_button").addEventListener("click", writeAPI);
        const vscode = acquireVsCodeApi();	
        function translateInput() {
            const input = document.getElementById('inputField').value;
			document.getElementById('status').innerText = "Running Translate...";
            vscode.postMessage({
                command: 'translateInput',
                input: input
            });
        }
        function writeAPI() {
        	const apikey = document.getElementById('keyField').value;
			document.getElementById('status').innerText = "Running Write API...";
            vscode.postMessage({
                command: 'writeAPI',
                input: apikey
            });
        }
        window.addEventListener('message', event => {
                    const message = event.data;
                    switch (message.command){
                        case 'received_content':
                            const fileContent = message.content;
                            document.getElementById('outputField').value = fileContent;
                            break;
						case 'received_status':
							const content = message.content;
							document.getElementById('status').innerText = content;
							break;
                    }
                });
    </script>
</body>
</html>
    `;
    webviewView.webview.onDidReceiveMessage(
			message => {
				switch (message.command) {
					case 'translateInput':
						// vscode.window.showErrorMessage(message.input);
						saveInput(message.input);
						// sendtranslateStatus(webviewView.webview);
						// call the python executable file
						callpython(webviewView.webview);
						printOutput(webviewView.webview);
						return;
					case 'writeAPI':
						sendstatus("Saving API key ...", webviewView.webview);
						saveAPI(message.input);
						sendstatus("Saving API key success.", webviewView.webview);
						return;
				}
			}, 
			undefined
			);
	}
  }