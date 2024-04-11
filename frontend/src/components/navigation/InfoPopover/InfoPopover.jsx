import { useEffect, useRef, useState } from "react";
import { Button, Overlay, OverlayTrigger, Popover, Tooltip } from "react-bootstrap";
import './InfoPopover.css';
import { useDispatch, useSelector } from "react-redux";
import { reset } from "../../../_store/errorSlice";

export default function InfoPopover() {
    const target = useRef(null);
    const err = useSelector((state) => state.error.error);
    const err_postfix = useSelector((state) => state.error.type)=="error"?"-err":"";
    const dispatch = useDispatch();
    useEffect(() => {
        if (err!=null) {
            const timer = setTimeout(()=>{dispatch(reset())},2000);
        }
    },[err])
    const errText = (err!=null)?err:"";
    return (
        <>
            <div ref={target}></div>
            <Overlay className="popup-overlay" target={target.current} show={err!=null} placement="bottom">
                {(props) => (
                    <Popover id="popover-basic" className={"popup"+err_postfix}>
                        <Popover.Header className={"popup-header"+err_postfix} as="h3">Error</Popover.Header>
                        <Popover.Body className="popup-body">
                            {errText}
                        </Popover.Body>
                    </Popover>
                )}
            </Overlay>
        </>
    )
}
