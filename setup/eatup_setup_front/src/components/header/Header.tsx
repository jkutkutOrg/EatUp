import appLogo from '../../assets/img/eatup_logo.png';
import ghLogo from '../../assets/img/github.png';
import Status from '../../model/Status';
import SetupApi from '../../services/SetupApi';

interface Props {
  onRefresh: () => void;
  status: Status;
}

function Header({status, onRefresh}: Props) {
  const staffHref = SetupApi.getStaffAppUrl();

  return <header className='sticky-top'>
    <div className="navbar bg-dark justify-content-between" style={{
      padding: "0px 9px",
    }}>
      <div className="col">
        <img src={appLogo} alt="Logo" width="42" height="42" onClick={onRefresh}/>
      </div>
      <div className="col text-center">
        {status == Status.Installed &&
          <a href={staffHref} target='_blank' style={{
            color: "white",
            cursor: "pointer"
          }}>Staff App</a>
        }
      </div>
      <div className="col text-end">
        <a href="https://github.com/jkutkutorg/EatUp" target='_blank' className='ml-auto p-2' style={{cursor: "pointer"}}>
          <img src={ghLogo} alt="GitHub" width="30" height="30"/>
        </a>
      </div>
    </div>
  </header>
}

export default Header;