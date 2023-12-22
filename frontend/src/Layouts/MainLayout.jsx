import { Outlet } from "react-router-dom";
import NavBar from "../components/navigation/NavBar/NavBar";
import InfoPopover from "../components/navigation/InfoPopover/InfoPopover";

export default function MainLayout(props) {
    return (
        <div>
            <InfoPopover errorState={props.errorState} />
            <NavBar {...props}/>
            <Outlet/>
        </div>
    )
}