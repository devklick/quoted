import { input } from "@inquirer/prompts";

const knownArgs: Record<keyof CliParams, { position: number }> = {
  name: {
    position: 0,
  },
} as const;

type CliParams = {
  name: string;
};

async function getParams(): Promise<CliParams> {
  const args = getParamsFromArgs();
  if (args) return args;

  return await promptForArgs();
}

function getParamsFromArgs(): CliParams | undefined {
  const knownArgArray = Object.entries(knownArgs).map(([name, value]) => ({
    name,
    ...value,
  }));
  const args = {};

  for (let position = 2; position < process.argv.length; position += 2) {
    const knownArg = knownArgArray.find((a) => a.position === position);
    if (knownArg) {
      args[knownArg.name] = process.argv[position];
    }
  }
  return args as CliParams;
}

async function promptForArgs(): Promise<CliParams> {
  const name = await input({
    message: "Component name (e.g. MyComponent):",
    // is PascalCase, ignoring the .tsx file type if it's present
    validate: (value) =>
      !!value.replace(/\.tsx$/, "").match(/^[A-Z][a-z]+(?:[A-Z][a-z]+)*$/),
    required: true,
    transformer: (value) => value.replace(/\.*$/, ""),
  });
  return { name };
}

const params = await getParams();
