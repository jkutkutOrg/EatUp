import appLogo from '../../assets/img/eatup_logo.png';
import ghLogo from '../../assets/img/github.png';

interface Props {
    onRefresh: () => void;
}

function Header({onRefresh}: Props) {
    return <header>
        <nav className="navbar bg-dark d-flex" data-bs-theme="dark">
            <div className='p-2' onClick={onRefresh} style={{cursor: "pointer"}}>
                <img src={appLogo} alt="GitHub" width="42" height="42"/>
            </div>
            <a href="https://github.com/jkutkutorg/EatUp" target='_blank' className='ml-auto p-2' style={{cursor: "pointer"}}>
                <img src={ghLogo} alt="GitHub" width="30" height="30"/>
            </a>
        </nav>
    </header>
}

export default Header;