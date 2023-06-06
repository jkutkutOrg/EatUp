import landingImg from '../assets/landing/landing.png'
import logo from '../assets/logo.png'

interface Props {
    onBegin: () => void;
}

const Landing = ({onBegin}: Props) => {
    return <div className='container-fluid eatup-landing  text-center'>
        <div className='row'>
            <div className='col-8'>
                <h2 className='landing-header'>Satisfy your cravings with a tap!</h2>
            </div>
            <div className='col-3'>
                <img className="landing-logo" src={logo} alt="logo" />
            </div>
        </div>
        <br />
        <br />
        <div className='row'>
            <div className='col'>
                <img className="landing-img" src={landingImg} alt="landing image" />
            </div>
        </div>
        <br />
        <br />
        <div className='row'>
            <div className='col'>
                <button className='btn btn-primary btn-lg landing-btn' onClick={onBegin}>Begin</button>
            </div>
        </div>
    </div>;
}

export default Landing;