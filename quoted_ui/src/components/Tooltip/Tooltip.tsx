import { useState, useRef, useLayoutEffect, useEffect, ReactNode } from "react";
import { createPortal } from "react-dom";
import styles from "./Tooltip.module.scss";
import clsx from "clsx";

interface TooltipProps {
  content: ReactNode;
  children: ReactNode;
  showDelay?: number;
  hideDelay?: number;
}

export default function Tooltip({
  content,
  children,
  showDelay = 500,
  hideDelay = 200,
}: TooltipProps) {
  const triggerRef = useRef<HTMLDivElement>(null);
  const tooltipRef = useRef<HTMLDivElement>(null);

  const showTimeout = useRef<number | null>(null);
  const hideTimeout = useRef<number | null>(null);

  const [visible, setVisible] = useState(false);
  const [style, setStyle] = useState({ top: 0, left: 0 });

  // ---- Delay logic ----
  function handleMouseEnter() {
    // cancel pending hide
    if (hideTimeout.current) {
      clearTimeout(hideTimeout.current);
      hideTimeout.current = null;
    }

    showTimeout.current = window.setTimeout(() => {
      setVisible(true);
    }, showDelay);
  }

  function handleMouseLeave() {
    // cancel pending show
    if (showTimeout.current) {
      clearTimeout(showTimeout.current);
      showTimeout.current = null;
    }

    hideTimeout.current = window.setTimeout(() => {
      setVisible(false);
    }, hideDelay);
  }

  // ---- Positioning logic ----
  useLayoutEffect(() => {
    if (!visible || !triggerRef.current || !tooltipRef.current) return;

    const triggerRect = triggerRef.current.getBoundingClientRect();
    const tooltipRect = tooltipRef.current.getBoundingClientRect();

    const margin = 8;

    let top = triggerRect.top + window.scrollY - tooltipRect.height - margin;
    let left =
      triggerRect.left +
      window.scrollX +
      triggerRect.width / 2 -
      tooltipRect.width / 2;

    // Clamp horizontally
    left = Math.max(
      margin,
      Math.min(left, window.innerWidth - tooltipRect.width - margin),
    );

    // Flip if needed
    if (top < margin) {
      top = triggerRect.bottom + window.scrollY + margin;
    }

    setStyle({ top, left });
  }, [visible, content]);

  // ---- Recalculate on scroll/resize ----
  useEffect(() => {
    if (!visible) return;

    const update = () => {
      if (!triggerRef.current || !tooltipRef.current) return;

      const triggerRect = triggerRef.current.getBoundingClientRect();
      const tooltipRect = tooltipRef.current.getBoundingClientRect();

      const margin = 8;

      let top = triggerRect.top + window.scrollY - tooltipRect.height - margin;
      let left =
        triggerRect.left +
        window.scrollX +
        triggerRect.width / 2 -
        tooltipRect.width / 2;

      left = Math.max(
        margin,
        Math.min(left, window.innerWidth - tooltipRect.width - margin),
      );

      if (top < margin) {
        top = triggerRect.bottom + window.scrollY + margin;
      }

      setStyle({ top, left });
    };

    window.addEventListener("scroll", update);
    window.addEventListener("resize", update);

    return () => {
      window.removeEventListener("scroll", update);
      window.removeEventListener("resize", update);
    };
  }, [visible]);

  // ---- Cleanup ----
  useEffect(() => {
    return () => {
      if (showTimeout.current) clearTimeout(showTimeout.current);
      if (hideTimeout.current) clearTimeout(hideTimeout.current);
    };
  }, []);

  return (
    <>
      <div
        ref={triggerRef}
        className={styles.trigger}
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
      >
        {children}
      </div>

      {visible &&
        createPortal(
          <div
            ref={tooltipRef}
            className={clsx(styles.tooltip, {
              [styles["tooltip--visible"]]: visible,
            })}
            style={style}
          >
            {content}
          </div>,
          document.body,
        )}
    </>
  );
}
