import { SelectItemObject } from "./Select";

export function isSelectedItemObject(item: unknown): item is SelectItemObject {
  return (
    item != null &&
    typeof item === "object" &&
    "value" in item &&
    "label" in item
  );
}
