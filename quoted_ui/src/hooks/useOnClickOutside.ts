import { RefObject, useEffect } from "react";

export function useOnClickOutside(
  ref: RefObject<HTMLDivElement | null>,
  callback: (event: MouseEvent | TouchEvent) => void,
) {
  useEffect(() => {
    function handleEvent(event: MouseEvent | TouchEvent) {
      // Do nothing if clicking inside the element
      if (!ref.current || ref.current.contains(event.target as Node)) {
        return;
      }

      callback(event);
    }

    document.addEventListener("mousedown", handleEvent);
    document.addEventListener("touchstart", handleEvent);

    return () => {
      document.removeEventListener("mousedown", handleEvent);
      document.removeEventListener("touchstart", handleEvent);
    };
  }, [ref, callback]);
}
