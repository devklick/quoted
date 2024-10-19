import styles from "./Loading.module.scss";

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
interface LoadingProps {}

// eslint-disable-next-line no-empty-pattern
function Loading({}: LoadingProps) {
  return <div className={styles["loading"]} />;
}

export default Loading;
