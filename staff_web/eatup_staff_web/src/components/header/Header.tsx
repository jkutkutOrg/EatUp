import appLogo from '../../assets/logo.png';
import ghLogo from '../../assets/header/github.png';
import HeaderOption from '../../model/header/HeaderOption';
import Menu from '../../model/App/Menu';

interface Props {
    onRefresh: () => void;
    onClose: () => void;
    menu: Menu;
    extraOptions?: HeaderOption[];
}

function Header({onRefresh, onClose, menu, extraOptions}: Props) {
    extraOptions = extraOptions || [];
    let closeBtn: boolean = menu != Menu.Tables && menu != Menu.Landing;
    return <header className='sticky-top'>
        <nav className="navbar bg-dark d-flex" data-bs-theme="dark" style={{
            padding: "0px 9px",
        }}>
            <div className='p-2' onClick={onRefresh} style={{cursor: "pointer"}}>
                <img src={appLogo} alt="GitHub" width="42" height="42"/>
            </div>
            {extraOptions.map((option) => (
                <div key={option.label}
                    className='p-2' data-bs-theme="dark" style={{cursor: "pointer"}}
                    onClick={option.onClick}
                >
                    {option.label}
                </div>
            ))}
            {!closeBtn &&
                <a href="https://github.com/jkutkutorg/EatUp" target='_blank' className='ml-auto p-2' style={{cursor: "pointer"}}>
                    <img src={ghLogo} alt="GitHub" width="30" height="30"/>
                </a>
            }
            {closeBtn && <div
                className='p-2' style={{cursor: "pointer", color: "white"}}
                onClick={onClose}
            >
                Close
            </div>}
        </nav>
    </header>
}

export default Header;