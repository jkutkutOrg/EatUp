import landingImg from '../assets/landing/landing.png'

interface Props {
  onBegin: () => void;
}

const Landing = ({onBegin}: Props) => {
  return <div className='container-fluid eatup-landing text-center'>
    <div className='row'>
      <div className='col'>
        <h2 className='landing-header'>Satisfy your clients cravings with a tap!</h2>
      </div>
    </div>
    <div className='row'>
      <div className='col'>
        <img className="landing-img img-fluid" src={landingImg} alt="landing image" />
      </div>
    </div>
    <div className='row'>
      <div className='col'>
        <button className='btn btn-primary btn-lg landing-btn' onClick={onBegin}>Begin</button>
      </div>
    </div>
  </div>;
}

export default Landing;