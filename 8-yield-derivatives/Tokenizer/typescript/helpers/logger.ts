import { Logger } from 'tslog'

export type AppLogger = typeof logger
export const logger = new Logger({
  minLevel: 0,
  prettyLogTemplate: '{{hh}}:{{MM}}:{{ss}}:{{ms}}\t{{logLevelName}}\t',
})
