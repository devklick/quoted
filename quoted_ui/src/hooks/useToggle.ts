import { useState } from "react";

interface UseToggleParams {
  defaultValue?: boolean;
}
type ToggleFn = () => void;
type UseToggleReturn = [boolean, { toggle: ToggleFn }];

function useToggle(params?: UseToggleParams): UseToggleReturn {
  const [value, setValue] = useState(params?.defaultValue ?? false);
  function toggle() {
    setValue(!value);
  }
  return [value, { toggle }];
}

export default useToggle;
