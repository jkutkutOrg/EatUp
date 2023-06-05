import landingImg from '../assets/landing/landing.png'

interface Props {
    onBegin: () => void;
}

const Landing = ({onBegin}: Props) => {
    return <>
        <h1>Satisfy your cravings with a tap!</h1>
        <img className="landing-img" src={landingImg} alt="landing image" />
        <button onClick={onBegin}>Begin</button>
    </>;
}

export default Landing;