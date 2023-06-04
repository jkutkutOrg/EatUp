import landingImg from '../assets/landing/landing.png'
import Header from "../components/header/Header";

interface Props {
    onHeaderRefresh: () => void;
    onBegin: () => void;
}

const Landing = ({onHeaderRefresh, onBegin}: Props) => {
    return <>
        <Header onRefresh={onHeaderRefresh} />
        <h1>Satisfy your cravings with a tap!</h1>
        <img className="landing-img" src={landingImg} alt="landing image" />
        <button onClick={onBegin}>Begin</button>
    </>;
}

export default Landing;