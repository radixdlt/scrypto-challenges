import decode_hex from './decode_hex.js'
   
  interface StepPosition {
    step: number;
    liquidity: number;
    last_stable_fees_per_liq: number;
    last_other_fees_per_liq: number;
  }
  
  interface Position {
    other_token: string;
    step_positions: StepPosition[];
  }
  
  function parseOutput(output: string): Position {
    const [otherToken, ...stepPositionsStrings] = output.trim().split("@");
    const stepPositions: StepPosition[] = [];
  
    for (const stepPositionString of stepPositionsStrings) {
      const [stepString, liquidityString, lastStableFeesPerLiqString, lastOtherFeesPerLiqString] = stepPositionString.split(" ");
      if (stepString == undefined || liquidityString == undefined || lastStableFeesPerLiqString == undefined || lastOtherFeesPerLiqString == undefined) {
        throw new Error("Undefined property")
      }
      const step = parseInt(stepString);
      const liquidity = parseFloat(liquidityString);
      const lastStableFeesPerLiq = parseFloat(lastStableFeesPerLiqString);
      const lastOtherFeesPerLiq = parseFloat(lastOtherFeesPerLiqString);
  
      stepPositions.push({ step, liquidity, last_stable_fees_per_liq: lastStableFeesPerLiq, last_other_fees_per_liq: lastOtherFeesPerLiq });
    }

    if (otherToken == undefined) {throw new Error("Undefined property")}
  
    return { other_token: otherToken, step_positions: stepPositions };
  }
  
  
export default async function decode_position(mutable_data_hex:string,immutable_data_hex:string): Promise<Position> {
    try {

      const decoded_data = await decode_hex(2,mutable_data_hex, immutable_data_hex)
  
      return Promise.resolve(parseOutput(decoded_data.stdout))

    } catch (e) {
      console.log(e)
      return Promise.reject(e)
    }
  }
  
