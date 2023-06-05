import landingImg from '../assets/landing/landing.png'

interface Props {
    onBegin: () => void;
}

const Landing = ({onBegin}: Props) => {
    return <div className='container text-center'>
        <div className='row'>
            <div className='col'>
                <h2 className='display-2'>Satisfy your cravings with a tap!</h2>
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
                <button className='btn btn-primary btn-lg' onClick={onBegin}>Begin</button>
            </div>
        </div>
    </div>;
}

export default Landing;