// import * as path from "path";
import { dirname } from 'path';
import { fileURLToPath } from 'url';
import { randomInt } from 'crypto'
import { $ as _$, fs, os, path, ProcessPromise } from "zx";

const __dirname = dirname(fileURLToPath(import.meta.url));

const envFilePath = path.resolve(__dirname, "../.env");

const _load_envs = () => fs.readFileSync(envFilePath, "utf-8").split(os.EOL);

const reset_env_vars = () => {
  fs.writeFileSync(envFilePath, '');
};

const get_env = (key: string, index: number | undefined = undefined): any => {


  let _key = typeof index === "number" ? `${key}_${index}` : key


  const matchedLine = _load_envs().find((line) => line.split("=")[0] === _key);


  return matchedLine !== undefined ? matchedLine.split("=")[1] : null;
};

const set_env = (key: string, value: string, _index: number | undefined = undefined) => {

  let _key = typeof _index === "number" ? `${key}_${_index}` : key

  const envVars = _load_envs();
  const targetLine = envVars.find((line) => line.split("=")[0] === _key);
  if (targetLine !== undefined) {
    // update existing line
    const targetLineIndex = envVars.indexOf(targetLine);
    // replace the key/value with the new value
    envVars.splice(targetLineIndex, 1, `${_key}=${value}`);
  } else {
    // create new key value
    envVars.push(`${_key}=${value}`);
  }
  // write everything back to the file system
  fs.writeFileSync(envFilePath, envVars.join(os.EOL));

  process.env[_key] = value;
};

const load_envs = () => {
  const envVars: string[] = _load_envs();
  for (let index = 0; index < envVars.length; index++) {
    let line = envVars[index].split('=')
    process.env[line[0]] = line[1];
  }
};

const regexResim = /(New Package|Account component address|Private key|Public key|└─ Component|├─ Component|└─ Resource|├─ Resource|NFAddress|NonFungibleGlobalId): ([\d|A-Za-z|_|:|#]+)/gm


async function exec_command(command: ProcessPromise, isQuiet = false) {

  try {
    let matches = (await (isQuiet ? command.quiet() : command)).stdout.matchAll(regexResim)
    let outputs = []
    for (const match of matches) {
      outputs.push(match[2])
    }
    return outputs
  } catch (error) {
    console.error(error)
  }

}

// async function resim(command: string, isQuiet = false) {

//   let new_command = `resim ${command}`

//   return exec_command(_$`${"" + new_command}`, isQuiet)
// }

async function _run_temp_manifest(manifest: any, name: string) {
  const envFilePath = path.resolve(__dirname, '../rtm/', name + ".rtm");
  fs.writeFileSync(envFilePath, manifest);

  let result = await exec_command(_$`resim run ${envFilePath}`);
  // fs.unlinkSync(envFilePath);
  return result;
}


export function getRandomFee() {
  return randomInt(5, 10) / 100
}

export { __dirname, _run_temp_manifest, get_env as get, set_env as set, reset_env_vars, exec_command as exe, load_envs };
