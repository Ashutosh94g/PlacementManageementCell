import { createSignal, Show } from "solid-js";
import { Portal } from "solid-js/web"
import { Link, useLocation, useNavigate } from "solid-app-router";

import { Store, Modal } from "../../utils";


function Header() {
  const navigate = useNavigate();
  const {getIsAuthed, logout, getDisplay, setDisplay} = Store;
  const [getShowModal, setShowModel] = createSignal(false);
  const location = useLocation();
  const modelLogoutHandler = () => {
    setShowModel(false);
    setDisplay(false);
    logout();
    navigate("/");
  }

  const logoutHandler = () => {
    setShowModel(true);
  }
  return (
  <header style={{position: "fixed", top: 0, width: "99vw"}}>
    <nav class="navbar navbar-expand-lg bg-success">
      <div class="container-fluid">
      <Link href="/" class="navbar-brand text-white">Placement</Link>
      <button class="navbar-toggler" type="button" id="Hamburger Drawer" onclick={() => setDisplay(v => !v)}>
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" classList={{"d-none": !getDisplay(), "d-flex": getDisplay()}} id="navbarSupportedContent">
      
      <Show when={getIsAuthed()} fallback={
        <>
        <ul class="navbar-nav me-auto mb-2 mb-lg-0">
          <li class="nav-item">
            <Link class="nav-link" classList={{ "text-white": location.pathname == "/"}} href="/">Home</Link>
          </li>
          {/* <li class="nav-item">
            <Link class="nav-link" classList={{ "text-white": location.pathname == "/about"}} href="/about">About</Link>
          </li> */}
        </ul>
        <div class="d-flex" style={{marginRight: "5px"}}>
          <Link href="/login">
          <button class="btn btn-success" classList={{ "active": location.pathname == "/login"}}>
            Login
          </button>
          </Link>
          <Link href="/register">
            <button class="btn btn-success" classList={{ "active": location.pathname == "/register"}}>
              Register
            </button>
          </Link>
        </div>
        </>
      }>
        <>
          <ul class="navbar-nav me-auto mb-2 mb-lg-0">
            <li class="nav-item">
              <Link class="nav-link" href="/" classList={{ "text-white": location.pathname == "/"}}>Home</Link>
            </li>
            <li class="nav-item">
              <Link class="nav-link" href="/add-category" classList={{ "text-white": location.pathname == "/add-category"}}>Add Category</Link>
            </li>
            <li class="nav-item">
              <Link class="nav-link" href="/filter" classList={{ "text-white": location.pathname == "/filter"}}>Filter</Link>
            </li>
          </ul>
          <div class="d-flex">
            <button class="btn btn-danger" onclick={logoutHandler}>Logout</button>
          </div>
          <Show
            when={getShowModal()}
            fallback={<></>}
          >
            <Portal>
              <Modal setter={setShowModel} title="Confirm Logout" body="click on the confirm button to logout" confirm={modelLogoutHandler} />
            </Portal>
          </Show>
        </>
      </Show>
        </div>
      </div>
    </nav>
  </header>
  );
}

export default Header;
