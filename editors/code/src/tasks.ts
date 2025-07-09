import * as vscode from "vscode";
import type { Config } from "./config";
import * as toolchain from "./toolchain";

// This ends up as the `type` key in tasks.json. RLS also uses `cargo` and
// our configuration should be compatible with it so use the same key.
export const CARGO_TASK_TYPE = "cargo";
export const SHELL_TASK_TYPE = "shell";

export const WGSL_TASK_SOURCE = "wgsl";

export type TaskDefinition = vscode.TaskDefinition & {
	readonly type: typeof CARGO_TASK_TYPE | typeof SHELL_TASK_TYPE;
	args?: string[];
	command: string;
};

export type CargoTaskDefinition = {
	env?: Record<string, string>;
	type: typeof CARGO_TASK_TYPE;
} & TaskDefinition;

function isCargoTask(definition: vscode.TaskDefinition): definition is CargoTaskDefinition {
	return definition.type === CARGO_TASK_TYPE;
}

class WgslTaskProvider implements vscode.TaskProvider {
	private readonly config: Config;

	constructor(config: Config) {
		this.config = config;
	}

	async provideTasks(): Promise<vscode.Task[]> {
		if (!vscode.workspace.workspaceFolders) {
			return [];
		}

		// Detect Rust tasks. Currently we do not do any actual detection
		// of tasks (e.g. aliases in .cargo/config) and just return a fixed
		// set of tasks that always exist. These tasks cannot be removed in
		// tasks.json - only tweaked.

		const task_definitions = [
			{ command: "build", group: vscode.TaskGroup.Build },
			{ command: "check", group: vscode.TaskGroup.Build },
			{ command: "clippy", group: vscode.TaskGroup.Build },
			{ command: "test", group: vscode.TaskGroup.Test },
			{ command: "clean", group: vscode.TaskGroup.Clean },
			{ command: "run", group: undefined },
		];

		// FIXME: The server should provide this
		const cargo = await toolchain.cargoPath();

		const tasks: vscode.Task[] = [];
		for (const workspaceTarget of vscode.workspace.workspaceFolders) {
			for (const task_definition of task_definitions) {
				const definition = {
					command: task_definition.command,
					type: CARGO_TASK_TYPE,
				} as const;
				const exec = await targetToExecution(definition, {}, cargo);
				const vscodeTask = await buildWgslTask(
					workspaceTarget,
					definition,
					`cargo ${task_definition.command}`,
					this.config.problemMatcher,
					exec,
				);
				vscodeTask.group = task_definition.group;
				tasks.push(vscodeTask);
			}
		}

		return tasks;
	}

	async resolveTask(task: vscode.Task): Promise<vscode.Task | undefined> {
		// VSCode calls this for every cargo task in the user's tasks.json,
		// we need to inform VSCode how to execute that command by creating
		// a ShellExecution for it.
		if (isCargoTask(task.definition)) {
			const exec = await targetToExecution(task.definition, { env: task.definition.env });
			return buildWgslTask(
				task.scope,
				task.definition,
				task.name,
				task.problemMatchers,
				exec,
			);
		}

		return undefined;
	}
}

export async function buildWgslTask(
	scope: vscode.WorkspaceFolder | vscode.TaskScope | undefined,
	definition: TaskDefinition,
	name: string,
	problemMatcher: string[],
	exec: vscode.ProcessExecution | vscode.ShellExecution,
): Promise<vscode.Task> {
	return new vscode.Task(
		definition,
		// scope can sometimes be undefined. in these situations we default to the workspace taskscope as
		// recommended by the official docs: https://code.visualstudio.com/api/extension-guides/task-provider#task-provider)
		scope ?? vscode.TaskScope.Workspace,
		name,
		WGSL_TASK_SOURCE,
		exec,
		problemMatcher,
	);
}

export async function targetToExecution(
	definition: TaskDefinition,
	options?: {
		env?: { [key: string]: string };
		cwd?: string;
	},
	cargo?: string,
): Promise<vscode.ProcessExecution | vscode.ShellExecution> {
	let command, args;
	if (isCargoTask(definition)) {
		// FIXME: The server should provide cargo
		command = cargo || (await toolchain.cargoPath(options?.env));
		args = [definition.command].concat(definition.args || []);
	} else {
		command = definition.command;
		args = definition.args || [];
	}
	return new vscode.ProcessExecution(command, args, options);
}

export function activateTaskProvider(config: Config): vscode.Disposable {
	const provider = new WgslTaskProvider(config);
	return vscode.tasks.registerTaskProvider(CARGO_TASK_TYPE, provider);
}
