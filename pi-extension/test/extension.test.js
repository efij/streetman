import assert from "node:assert/strict";
import test from "node:test";
import { parseStreetmanLeanCommand } from "../index.js";

test("parseStreetmanLeanCommand accepts supported modes", () => {
  assert.equal(parseStreetmanLeanCommand("ultra"), "ultra");
  assert.equal(parseStreetmanLeanCommand("off"), "off");
  assert.equal(parseStreetmanLeanCommand("???"), "full");
});
