import { Outlet } from "react-router-dom";
import NavBar from "../components/navigation/Header/NavBar";

export default function MainLayout() {
    return (
        <div>
            <NavBar/>
            <Outlet/>
        </div>
    )
}