import { RouteEvaluator, SwapHop } from "./route_evaluator";

export class SolanaJupiterRouteClient {
  public readonly evaluator: RouteEvaluator;

  constructor() {
    this.evaluator = new RouteEvaluator();
  }

  public verifyMinimumOutput(expected: bigint, actual: bigint, slippageBps: number): boolean {
    const minRequired = (expected * BigInt(10000 - slippageBps)) / 10000n;
    return actual >= minRequired;
  }
}
