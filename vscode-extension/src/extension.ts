import * as vscode from "vscode";
import { PracticeController } from "./practiceController";

export function activate(context: vscode.ExtensionContext): void {
  const controller = new PracticeController(vscode);
  controller.register(context);
}

export function deactivate(): void {}
