interface Props {
    ftCreate: () => void;
}

const Start = ({ftCreate}: Props) => {
    return <>
    <h1>Start</h1>
    <p>The project has not been created yet.</p>
    <button onClick={ftCreate}>Create</button>
  </>;
}

export default Start;