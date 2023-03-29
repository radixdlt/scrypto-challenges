import { exec } from 'child_process'

const DECODER = "./target/release/stoichiometric-decoder"


export default async function decode_hex(code:number, mutable_data_hex:string, immutable_data_hex:string):Promise<{ stdout: string, stderr: string }> {
    return new Promise((resolve, reject) => {
      exec(`${DECODER} ${code} ${mutable_data_hex} ${immutable_data_hex}`, (error, stdout, stderr) => {
        if (error) {
          reject(error);
        } else {
          resolve({ stdout, stderr });
        }
      });
    });
  }