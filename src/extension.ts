import * as vscode from 'vscode';


import { SidebarProvider } from "./utils/webviewUtils";


export function activate(context: vscode.ExtensionContext) {
	console.log('Congratulations, your extension "cstarsx" is now active!');
    // 注册一个命令，用于打开侧边栏中的输入窗口

	const sidebarPanel = new SidebarProvider(context);
	context.subscriptions.push(
		vscode.window.registerWebviewViewProvider('vs-sidebar-view', sidebarPanel)
	);
	
    // let disposable = vscode.commands.registerCommand('cstarsx.createInput', () => {
    //     // 创建一个 Webview 面板
    //     const panel = vscode.window.createWebviewPanel(
    //         'inputPanel', // 面板的唯一标识符
    //         'Input Panel', // 面板的标题
    //         vscode.ViewColumn.One, // 面板要显示的位置
    //         {
    //             enableScripts: true // 允许执行 JavaScript
    //         } // 面板的其他属性
    //     );
		
    //     // 加载 HTML 内容到 Webview 面板中
    //     panel.webview.html = getWebviewContent(panel.webview);
	// 	// 接收来自 Webview 的消息
	// 	panel.webview.onDidReceiveMessage(
	// 		message => {
	// 			switch (message.command) {
	// 				case 'saveInput':
	// 					// vscode.window.showErrorMessage(message.input);
	// 					saveInput(message.input);
	// 					// call the python executable file
	// 					callpython();
	// 					printOutput(panel.webview);
	// 					return;
	// 				case 'refresh':
	// 					printOutput(panel.webview);
	// 					return;
	// 			}
	// 		}, 
	// 		undefined,
	// 		context.subscriptions
	// 		);
	// });

    // context.subscriptions.push(disposable);
	console.log("activate function done.");
}
