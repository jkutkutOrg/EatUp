interface Props {
    onRefresh: () => void;
}

function Header({onRefresh}: Props) {
    return <header>
        <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
            <div className="container-fluid">
            <button
                className="navbar-toggler"
                type="button"
                data-mdb-toggle="collapse"
                data-mdb-target="#headerNavbar"
                aria-controls="headerNavbar"
                aria-expanded="false"
                aria-label="Toggle navigation"
            >
                <i className="fas fa-bars"></i>
            </button>
            <div className="collapse navbar-collapse" id="headerNavbar">
                <ul className="navbar-nav me-auto mb-2 mb-lg-0">
                    {/* TODO active CSS class */}
                    <li className="nav-item">
                        <a className="nav-link" href="#">Services Handler</a>
                    </li>
                    <li className="nav-item">
                        <button className="nav-link btn btn-link" onClick={onRefresh}>Refresh</button>
                    </li>
                </ul>
            </div>
            </div>
        </nav>
    </header>
}

export default Header;