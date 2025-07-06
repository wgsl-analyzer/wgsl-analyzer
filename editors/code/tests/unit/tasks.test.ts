import type { Context } from ".";
import * as vscode from "vscode";
import * as assert from "assert";
import { targetToExecution } from "../../src/tasks";

export async function getTests(ctx: Context) {
	await ctx.suite("Tasks", (suite) => {
		suite.addTest("cargo targetToExecution", async () => {
			assert.deepStrictEqual(
				await targetToExecution({
					type: "cargo",
					command: "check",
					arguments: ["foo"],
				}).then(executionToSimple),
				{
					process: "cargo",
					arguments: ["check", "foo"],
				},
			);
		});

		suite.addTest("shell targetToExecution", async () => {
			assert.deepStrictEqual(
				await targetToExecution({
					type: "shell",
					command: "thing",
					arguments: ["foo"],
				}).then(executionToSimple),
				{
					process: "thing",
					arguments: ["foo"],
				},
			);
		});

		suite.addTest("base tasks", async () => {
			const tasks = await vscode.tasks.fetchTasks({ type: "cargo" });
			const expectedTasks = [
				{
					definition: { type: "cargo", command: "build" },
					name: "cargo build",
					execution: {
						process: "cargo",
						arguments: ["build"],
					},
				},
				{
					definition: {
						type: "cargo",
						command: "check",
					},
					name: "cargo check",
					execution: {
						process: "cargo",
						arguments: ["check"],
					},
				},
				{
					definition: { type: "cargo", command: "clippy" },
					name: "cargo clippy",
					execution: {
						process: "cargo",
						arguments: ["clippy"],
					},
				},
				{
					definition: { type: "cargo", command: "test" },
					name: "cargo test",
					execution: {
						process: "cargo",
						arguments: ["test"],
					},
				},
				{
					definition: {
						type: "cargo",
						command: "clean",
					},
					name: "cargo clean",
					execution: {
						process: "cargo",
						arguments: ["clean"],
					},
				},
				{
					definition: { type: "cargo", command: "run" },
					name: "cargo run",
					execution: {
						process: "cargo",
						arguments: ["run"],
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
		arguments: string[];
	} & ({ command: string } | { process: string });
} {
	const execution = executionToSimple(task.execution!);

	return {
		definition: task.definition,
		name: task.name,
		execution,
	};
}

function executionToSimple(
	taskExecution: vscode.ProcessExecution | vscode.ShellExecution | vscode.CustomExecution,
): {
	arguments: string[];
} & ({ command: string } | { process: string }) {
	const exec = taskExecution as vscode.ProcessExecution | vscode.ShellExecution;
	if (exec instanceof vscode.ShellExecution) {
		return {
			command: typeof exec.command === "string" ? exec.command : (exec.command?.value ?? ""),
			arguments: (exec.args ?? []).map((arg) => {
				if (typeof arg === "string") {
					return arg;
				}
				return arg.value;
			}),
		};
	} else {
		return {
			process: exec.process,
			arguments: exec.args,
		};
	}
}
