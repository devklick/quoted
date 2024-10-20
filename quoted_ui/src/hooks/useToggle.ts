import { useState } from "react";

interface UseToggleParams {
  defaultValue?: boolean;
}
type ToggleFn = () => void;
type SetToggledFn = (value: boolean) => void;
type UseToggleReturn = [
  boolean,
  { toggle: ToggleFn; setToggled: SetToggledFn }
];

function useToggle(params?: UseToggleParams): UseToggleReturn {
  const [value, setValue] = useState(params?.defaultValue ?? false);
  function toggle() {
    setValue(!value);
  }
  function setToggled(value: boolean) {
    setValue(value);
  }
  return [value, { toggle, setToggled }];
}

export default useToggle;
