import { Outlet } from "react-router-dom";
import NavBar from "../../components/navigation/NavBar/NavBar";
import InfoPopover from "../../components/navigation/InfoPopover/InfoPopover";
import { useState } from "react";
import "./MainLayout.css";

export default function MainLayout() {
    const [headerHeight, passHeaderHeight] = useState(0);
    const queryHook = useState(false);
    return (
        <div>
            <InfoPopover/>
            <NavBar passHeaderHeight={passHeaderHeight} reset={()=> queryHook[1](true)}/>
            <Outlet context={{headerHeight:headerHeight,queryHook:queryHook}}/>
        </div>
    )
}