import { useEffect, useRef, useState } from "react";
import clsx from "clsx";

import useToggle from "../../hooks/useToggle";

import styles from "./Select.module.scss";
import { useOnClickOutside } from "../../hooks/useOnClickOutside";
import Tooltip from "../Tooltip/Tooltip";

export interface SelectItemObject {
  value: string;
  label: string;
}
export type SelectItem = SelectItemObject | string;

function getSelectItemValue(item: SelectItem): string {
  return typeof item === "string" ? item : item.value;
}

function getSelectItemLabel(item: SelectItem): string {
  return typeof item === "string" ? item : item.label;
}

interface SelectProps {
  value?: string;
  placeholder?: string;
  items: Array<SelectItem>;
  onChanged(item: SelectItem | undefined): void;
  className?: string;
  paginated?: boolean;
  hasMore?: boolean;
  loadMore?(): void;
  loading?: boolean;
  disabled?: boolean;
}

export default function Select({
  items,
  onChanged,
  value,
  className,
  paginated,
  loadMore,
  hasMore,
  loading,
  disabled,
  placeholder,
}: SelectProps) {
  const [open, { toggle: toggleOpen, setToggled: setOpen }] = useToggle();
  const ref = useRef<HTMLDivElement>(null);
  const [selected, setSelected] = useState<SelectItem | undefined>(
    (value && items.find((i) => getSelectItemValue(i) === value)) || undefined,
  );

  useEffect(() => {
    const selectedValue = selected && getSelectItemValue(selected);
    if (selectedValue !== value) {
      setSelected(value);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [value]);

  useOnClickOutside(ref, () => setOpen(false));

  function handleToggle() {
    if (!disabled) {
      toggleOpen();
    }
  }

  function handleItemClicked(item: SelectItem) {
    if (!disabled) {
      const newValue = item === selected ? undefined : item;

      if (newValue) {
        setOpen(false);
      }

      setSelected(newValue);
      onChanged(newValue);
    }
  }

  return (
    <div
      className={clsx(styles["select"], className, {
        [styles["select--open"]]: open,
        [styles["select--closed"]]: !open,
        [styles["select--disabled"]]: disabled,
      })}
      ref={ref}
    >
      <div
        className={clsx(styles["select__input"], {
          [styles["select__input--placeholder"]]: !selected,
        })}
        onClick={handleToggle}
      >
        {(selected && getSelectItemLabel(selected)) ||
          placeholder ||
          "Select..."}
      </div>

      {open && (
        <div className={styles["select__items"]}>
          {items.map((item) => (
            <Tooltip content={getSelectItemLabel(item)}>
              <div
                className={clsx(styles["select__item"], {
                  [styles["select__item--selected"]]:
                    selected && getSelectItemValue(item) === selected,
                })}
                onClick={() => handleItemClicked(item)}
              >
                {getSelectItemLabel(item)}
              </div>
            </Tooltip>
          ))}
          {paginated && hasMore && (
            <div className={clsx(styles["select__item"])} onClick={loadMore}>
              Load more...
            </div>
          )}
          {loading && (
            <div className={clsx(styles["select__item"])}>loading...</div>
          )}
        </div>
      )}
    </div>
  );
}
