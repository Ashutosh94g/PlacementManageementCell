import { Link } from "solid-app-router";

function Footer() {
  return (
  <footer class="bg-light text-center text-lg-start" style={{position: "fixed", bottom: 0, width: "100vw"}}>
  <div class="text-center p-3" style="background-color: rgba(0, 0, 0, 0.2);">
    © 2022 Copyright:
    <Link class="text-dark" href="/"> GNDU Placement Cell</Link>
  </div>
</footer>
);
}

export default Footer;
