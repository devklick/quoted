import React from "react";
import clsx from "clsx";

import styles from "./Button.module.scss";

type Corners = Partial<{ tl: number; tr: number; bl: number; br: number }>;
type Edges = Partial<{ t: number; r: number; b: number; l: number }>;

function isCorners(value?: unknown): value is Corners {
  const corners = value as Corners;
  return [corners?.bl, corners?.br, corners?.tl, corners?.tr].some(
    (v) => v !== undefined
  );
}

function isEdges(value?: unknown): value is Edges {
  const edges = value as Edges;
  return [edges?.b, edges?.l, edges?.r, edges?.t].some((v) => v !== undefined);
}

interface ButtonProps {
  type: "primary" | "secondary" | "subtle" | "ghost";
  width?: number | string;
  height?: number | string;
  size?: number | string;
  radius?: number | Corners;
  disabled?: boolean;
  children?: React.ReactNode;
  padding?: number | Edges;
  border?: number | Edges;
  active?: boolean;
  onClick?(): void;
}

function Button({
  radius,
  type,
  children,
  disabled,
  height,
  size,
  width,
  border,
  active,
  padding,
  onClick,
}: ButtonProps) {
  const overrides: React.CSSProperties = {};
  if (size) {
    overrides.width = overrides.height = size;
  } else {
    if (height) {
      overrides.height = height;
    }
    if (width) {
      overrides.width = width;
    }
  }

  if (isCorners(radius)) {
    overrides.borderBottomLeftRadius = radius.bl;
    overrides.borderBottomRightRadius = radius.br;
    overrides.borderTopLeftRadius = radius.tl;
    overrides.borderTopRightRadius = radius.tr;
  } else if (radius !== undefined) {
    overrides.borderRadius = radius;
  }

  if (isEdges(border)) {
    overrides.borderTopWidth = border.t;
    overrides.borderBottomWidth = border.b;
    overrides.borderLeftWidth = border.l;
    overrides.borderRightWidth = border.r;
  } else if (border !== undefined) {
    overrides.borderWidth = border;
  }

  if (isEdges(padding)) {
    overrides.paddingTop = padding.t;
    overrides.paddingBottom = padding.b;
    overrides.paddingLeft = padding.l;
    overrides.paddingRight = padding.r;
  } else if (padding !== undefined) {
    overrides.padding = padding;
  }

  return (
    <button
      disabled={disabled}
      className={clsx({
        [styles[`button`]]: true,
        [styles[`button--active`]]: active,
        [styles[`button__${type}`]]: true,
        [styles[`button__${type}--active`]]: active,
        [styles[`button--disabled`]]: disabled,
        [styles[`button__${type}--disabled`]]: disabled,
      })}
      style={overrides}
      onClick={onClick}
    >
      {children}
    </button>
  );
}

export default Button;
