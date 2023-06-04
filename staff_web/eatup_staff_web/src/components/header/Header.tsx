import appLogo from '../../assets/logo.png';
import ghLogo from '../../assets/header/github.png';

interface HeaderOption {
    label: string;
    onClick: () => void;
}

interface Props {
    onRefresh: () => void;
    options?: HeaderOption[];
}

function Header({onRefresh, options}: Props) {
    options = options || [];
    return <header>
        <nav className="navbar bg-dark d-flex" data-bs-theme="dark" style={{
            padding: "0px 9px",
        }}>
            <div className='p-2' onClick={onRefresh} style={{cursor: "pointer"}}>
                <img src={appLogo} alt="GitHub" width="42" height="42"/>
            </div>
            {options.map((option) => (
                <div key={option.label}
                    className='p-2' style={{cursor: "pointer", color: "white"}}
                    onClick={option.onClick}
                >
                    {option.label}
                </div>
            ))}
            <a href="https://github.com/jkutkutorg/EatUp" target='_blank' className='ml-auto p-2' style={{cursor: "pointer"}}>
                <img src={ghLogo} alt="GitHub" width="30" height="30"/>
            </a>
        </nav>
    </header>
}

export default Header;