export interface SwapHop {
  dexName: string;
  inputMint: string;
  outputMint: string;
  expectedOut: bigint;
  feeBps: number;
}

export class RouteEvaluator {
  public findBestRoute(routes: SwapHop[][]): SwapHop[] {
    if (routes.length === 0) {
      throw new Error("No routes provided for evaluation");
    }

    return routes.reduce((best, curr) => {
      const bestNetOut = this.calculateNetOutput(best);
      const currNetOut = this.calculateNetOutput(curr);
      return currNetOut > bestNetOut ? curr : best;
    });
  }

  public calculateNetOutput(hops: SwapHop[]): bigint {
    if (hops.length === 0) return 0n;
    const finalHop = hops[hops.length - 1];
    const totalFeeBps = hops.reduce((acc, h) => acc + h.feeBps, 0);
    return (finalHop.expectedOut * BigInt(10000 - totalFeeBps)) / 10000n;
  }
}
