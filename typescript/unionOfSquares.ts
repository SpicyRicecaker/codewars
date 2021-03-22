import {
  assertEquals,
} from "https://deno.land/std/testing/asserts.ts";
// We're taking in two squares and their left and right values
class Rect {
  // Top left corner coords
  x: number;
  y: number;
  // Width and height
  w: number;
  h: number;
  // Constructor, we take in the bottom left and top right corner
  constructor(x: number, y: number, w: number, h: number) {
    this.x = x;
    this.y = y;
    this.w = w;
    this.h = h;
  }

  // Takes in custom values
  static rectFromBlTr(l: [number, number], r: [number, number]) {
    // X is left x
    const x = l[0];
    // Y is right y
    const y = r[1];

    // Width is right x - x
    const w = r[0] - x;

    // Height is y - left y
    const h = y - l[1];
    return new Rect(x, y, w, h);
  }
  // Finds intersection of two squares!
  static findIntersectArea(rect1: Rect, rect2: Rect) {
    // Compare the x values
    // [1, 3], [2, 4], intersect [2, 3]
    //
    // [1, 4], [2, 3], intersect [2,3]
    //
    // So far max of x1, min of x2 works
    //
    // [1,2], [3, 4], intersect [3, 2]
    //
    // Min right max left, but make sure that it's above 0
    // console.log(rect1.x + rect1.w, rect2.x + rect2.w);
    // console.log(rect1.x, rect2.x);
    const width = Math.max(
      0,
      Math.min(rect1.x + rect1.w, rect2.x + rect2.w) -
        Math.max(rect1.x, rect2.x)
    );

    // Same thing with y basically, min of top max of bot
    const height = Math.max(
      0,
      Math.min(rect1.y, rect2.y) -
        Math.max(rect1.y - rect1.h, rect2.y - rect2.h)
    );

    // Return the final area
    return width * height;
  }
  get area(): number {
    return this.w * this.h;
  }
  // Finds union of two squares
  static findUnionArea(rect1: Rect, rect2: Rect) {
    // Easiest way is to sum areas then subtract the intersect once
    return rect1.area + rect2.area - Rect.findIntersectArea(rect1, rect2);
  }
}

Deno.test("second example", () => {
  const l1: [number, number] = [2, 1];
  const r1: [number, number] = [5, 5];
  const l2: [number, number] = [3, 2];
  const r2: [number, number] = [5, 7];

  const rect1 = Rect.rectFromBlTr(l1, r1);
  const rect2 = Rect.rectFromBlTr(l2, r2);
    
  assertEquals(16, Rect.findUnionArea(rect1, rect2));
})

Deno.test("first example", () => {
  const l1: [number, number] = [2, 2];
  const r1: [number, number] = [5, 7];
  const l2: [number, number] = [3, 4];
  const r2: [number, number] = [6, 9];

  const rect1 = Rect.rectFromBlTr(l1, r1);
  const rect2 = Rect.rectFromBlTr(l2, r2);
    
  assertEquals(24, Rect.findUnionArea(rect1, rect2));
})