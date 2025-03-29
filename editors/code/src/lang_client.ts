import * as lc from "vscode-languageclient/node";
import * as vscode from "vscode";

export class WaLanguageClient extends lc.LanguageClient {
	override handleFailedRequest<T>(
		type: lc.MessageSignature,
		token: vscode.CancellationToken | undefined,
		// eslint-disable-next-line @typescript-eslint/no-explicit-any
		error: any,
		defaultValue: T,
		showNotification?: boolean | undefined,
	): T {
		const showError = vscode.workspace
			.getConfiguration("wgsl-analyzer")
			.get("showRequestFailedErrorNotification");
		if (
			!showError &&
			error instanceof lc.ResponseError &&
			error.code === lc.ErrorCodes.InternalError
		) {
			// Do not show notification for internal errors, these are emitted by w-a when a request fails.
			showNotification = false;
		}

		return super.handleFailedRequest(type, token, error, defaultValue, showNotification);
	}
}
