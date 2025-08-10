import * as vscode from "vscode";

import type { Config } from "./config";

import * as toolchain from "./toolchain";

// This ends up as the `type` key in tasks.json. RLS also uses `cargo` and
// our configuration should be compatible with it so use the same key.
export const WESL_TASK_TYPE = "wesl";
export const SHELL_TASK_TYPE = "shell";

export const WESL_TASK_SOURCE = "wesl";

export type TaskDefinition = vscode.TaskDefinition & {
	readonly type: typeof SHELL_TASK_TYPE | typeof WESL_TASK_TYPE;
	args?: string[];
	command: string;
};

export type WeslTaskDefinition = TaskDefinition & {
	env?: Record<string, string>;
	type: typeof WESL_TASK_TYPE;
};

function isWeslTask(definition: vscode.TaskDefinition): definition is WeslTaskDefinition {
	return definition.type === WESL_TASK_TYPE;
}

class WeslTaskProvider implements vscode.TaskProvider {
	private readonly config: Config;

	constructor(config: Config) {
		this.config = config;
	}

	async provideTasks(): Promise<vscode.Task[]> {
		if (!vscode.workspace.workspaceFolders) {
			return [];
		}

		// Available WESL tasks.
		// Currently we do not do any actual detection of tasks, for example, aliases in `.wesl/config`.
		// Instead, this is the set of tasks that always exist.
		// These tasks cannot be removed in tasks.json - only tweaked.
		//
		// Sourced from:
		// https://github.com/wgsl-tooling-wg/wesl-rs/blob/main/crates/wesl-cli/src/main.rs#L45
		const task_definitions = [
			{ command: "check", group: vscode.TaskGroup.Build },
			{ command: "compile", group: vscode.TaskGroup.Build },
			{ command: "eval", group: vscode.TaskGroup.Test },
			{ command: "exec", group: undefined },
			{ command: "package", group: undefined },
			// { command: "glinty", group: vscode.TaskGroup.Build },
			// { command: "test", group: vscode.TaskGroup.Test },
			// { command: "clean", group: vscode.TaskGroup.Clean },
		];

		// FIXME: The server should provide this
		const cargo = await toolchain.weslPath();

		const tasks: vscode.Task[] = [];
		for (const workspaceTarget of vscode.workspace.workspaceFolders) {
			for (const task_definition of task_definitions) {
				const definition = {
					command: task_definition.command,
					type: WESL_TASK_TYPE,
				} as const;
				const exec = await targetToExecution(definition, {}, cargo);
				const vscodeTask = buildWeslTask(
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

	async resolveTask(task: vscode.Task): Promise<undefined | vscode.Task> {
		// VSCode calls this for every cargo task in the user's tasks.json,
		// we need to inform VSCode how to execute that command by creating
		// a ShellExecution for it.
		if (isWeslTask(task.definition)) {
			const exec = await targetToExecution(task.definition, { env: task.definition.env });
			return buildWeslTask(
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

export function buildWeslTask(
	scope: undefined | vscode.TaskScope | vscode.WorkspaceFolder,
	definition: TaskDefinition,
	name: string,
	problemMatcher: string[],
	exec: vscode.ProcessExecution | vscode.ShellExecution,
): vscode.Task {
	return new vscode.Task(
		definition,
		// `scope` can be undefined.
		// In these situations, default to the workspace taskscope;
		// this is the documented recommendation:
		// https://code.visualstudio.com/api/extension-guides/task-provider#task-provider
		scope ?? vscode.TaskScope.Workspace,
		name,
		WESL_TASK_SOURCE,
		exec,
		problemMatcher,
	);
}

export async function targetToExecution(
	definition: TaskDefinition,
	options?: {
		cwd?: string;
		env?: { [key: string]: string };
	},
	cargo?: string,
): Promise<vscode.ProcessExecution | vscode.ShellExecution> {
	let command, args;
	if (isWeslTask(definition)) {
		// FIXME: The server should provide wesl-rs
		command = cargo || (await toolchain.weslPath(options?.env));
		args = [definition.command].concat(definition.args || []);
	} else {
		command = definition.command;
		args = definition.args || [];
	}
	return new vscode.ProcessExecution(command, args, options);
}

export function activateTaskProvider(config: Config): vscode.Disposable {
	const provider = new WeslTaskProvider(config);
	return vscode.tasks.registerTaskProvider(WESL_TASK_TYPE, provider);
}
