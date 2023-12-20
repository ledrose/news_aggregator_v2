import { Outlet } from "react-router-dom";
import NavBar from "../components/navigation/NavBar/NavBar";

export default function MainLayout({userState}) {
    return (
        <div>
            <NavBar userState={userState}/>
            <Outlet/>
        </div>
    )
}