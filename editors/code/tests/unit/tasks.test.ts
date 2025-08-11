import * as assert from "assert";
import * as vscode from "vscode";
import { targetToExecution } from "../../src/tasks";
import type { Context } from ".";

export async function getTests(ctx: Context) {
	await ctx.suite("Tasks", (suite) => {
		suite.addTest("wesl targetToExecution", async () => {
			assert.deepStrictEqual(
				await targetToExecution({
					type: "wesl",
					command: "check",
					args: ["foo"],
				}).then(executionToSimple),
				{
					process: "wesl",
					args: ["check", "foo"],
				},
			);
		});

		suite.addTest("shell targetToExecution", async () => {
			assert.deepStrictEqual(
				await targetToExecution({
					type: "shell",
					command: "thing",
					args: ["foo"],
				}).then(executionToSimple),
				{
					process: "thing",
					args: ["foo"],
				},
			);
		});

		suite.addTest("base tasks", async () => {
			const tasks = await vscode.tasks.fetchTasks({ type: "wesl" });
			const expectedTasks = [
				{
					definition: { command: "compile", type: "wesl" },
					name: "wesl compile",
					execution: {
						process: "wesl",
						args: ["compile"],
					},
				},
				{
					definition: {
						type: "wesl",
						command: "check",
					},
					name: "wesl check",
					execution: {
						process: "wesl",
						args: ["check"],
					},
				},
				{
					definition: { command: "clippy", type: "wesl" },
					name: "wesl clippy",
					execution: {
						process: "wesl",
						args: ["clippy"],
					},
				},
				{
					definition: { command: "test", type: "wesl" },
					name: "wesl test",
					execution: {
						process: "wesl",
						args: ["test"],
					},
				},
				{
					definition: {
						type: "wesl",
						command: "clean",
					},
					name: "wesl clean",
					execution: {
						process: "wesl",
						args: ["clean"],
					},
				},
				{
					definition: { command: "exec", type: "wesl" },
					name: "wesl exec",
					execution: {
						process: "wesl",
						args: ["exec"],
					},
				},
			];
			tasks.map(to_test_execution).forEach((actual, i) => {
				const expected = expectedTasks[i];
				assert.deepStrictEqual(actual, expected);
			});
		});
	});
}

function to_test_execution(task: vscode.Task): {
	definition: vscode.TaskDefinition;
	name: string;
	execution: {
		args: string[];
	} & ({ command: string } | { process: string });
} {
	const execution = executionToSimple(task.execution);

	return {
		definition: task.definition,
		name: task.name,
		execution,
	};
}

function executionToSimple(
	taskExecution:
		| vscode.ProcessExecution
		| vscode.ShellExecution
		| vscode.CustomExecution
		| undefined,
): {
	args: string[];
} & ({ command: string } | { process: string }) {
	const exec = taskExecution as vscode.ProcessExecution | vscode.ShellExecution;
	if (exec instanceof vscode.ShellExecution) {
		return {
			command:
				typeof exec.command === "string"
					? exec.command
					: (exec.command?.value ?? ""),
			args: (exec.args ?? []).map((arg) => {
				if (typeof arg === "string") {
					return arg;
				}
				return arg.value;
			}),
		};
	} else {
		return {
			process: exec.process,
			args: exec.args,
		};
	}
}
