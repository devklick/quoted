import styles from "./Home.module.scss";

// eslint-disable-next-line @typescript-eslint/no-empty-object-type
interface HomeProps {}

// eslint-disable-next-line no-empty-pattern
function Home({}: HomeProps) {
  return (
    <div className={styles["home"]}>
      <h2>Get quotes from your favorite TV shows!</h2>
      <h3>What is this site?</h3>
      <p>
        Quoted allows you to search for quotes from your favorite TV shows,
        season and episodes. These could be anything from witty-one-liners to
        full-blown conversations between multiple characters.
      </p>
      <p>
        If you're not a fan of using a web interface or you would rather achieve
        the same thing without leaving the comfort of your keyboard, you can
        check out the quoted CLI application (find it on Github using the link
        in the header). Note that the CLI isn't published yet, so you'd need to
        run it from source.
      </p>
      <h3>
        What <i>could</i> this site be?
      </h3>
      <p>
        It would make sense to expand Quoted to cover Movie quotes as well. This
        will likely be added in the future.
      </p>
      <p>
        It's possible that, one day, you may be able to log in catalogue your
        own quotes etc, but in all honestly this probably wont happen. For now,
        you'll have to make do with some of <i>my</i> favorite quotes.
      </p>
    </div>
  );
}

export default Home;
