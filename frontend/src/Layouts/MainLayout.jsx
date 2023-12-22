import { Outlet } from "react-router-dom";
import NavBar from "../components/navigation/NavBar/NavBar";
import InfoPopover from "../components/navigation/InfoPopover/InfoPopover";

export default function MainLayout() {
    return (
        <div>
            <InfoPopover/>
            <NavBar/>
            <Outlet/>
        </div>
    )
}