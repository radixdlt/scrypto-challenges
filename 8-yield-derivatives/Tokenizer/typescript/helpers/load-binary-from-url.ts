import { ResultAsync } from 'neverthrow'
import fs from 'fs/promises'
import path from 'path'
import { typedError } from './typed-error'

const appDir = path.resolve('./')

export const loadBinaryFromPath = (path: string) =>
  ResultAsync.fromPromise(fs.readFile(`${appDir}${path}`), typedError)
