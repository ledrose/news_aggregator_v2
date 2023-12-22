import { useEffect, useRef, useState } from "react";
import { Button, Overlay, OverlayTrigger, Popover, Tooltip } from "react-bootstrap";
import './InfoPopover.css';
import { useDispatch, useSelector } from "react-redux";
import { reset } from "../../../_store/errorSlice";

export default function InfoPopover() {
    // const text = "Lorem ipsum fjwlefjwlekfjwlkfjewlkfjwlkfjwelkfjwelkfwjeflkwejflk";
    const target = useRef(null);
    const err = useSelector((state) => state.error.error);
    const dispatch = useDispatch();
    // const [err,setErr] = errorState;
    // const [show,setShow] = useState(false);
    useEffect(() => {
        if (err!=null) {
            const timer = setTimeout(()=>{dispatch(reset())},2000);
        }
    },[err])
    const errText = (err!=null)?err:"";
    return (
        <>
            {/* <Button ref={target} variant="primary" onClick={()=>setShow(true)}>Show modal</Button> */}
            <div ref={target}></div>
            <Overlay target={target.current} show={err!=null} placement="bottom">
                {(props) => (
                    <Popover id="popover-basic" className="popup-error">
                        <Popover.Header as="h3">Error</Popover.Header>
                        <Popover.Body>
                            {errText}
                        </Popover.Body>
                    </Popover>
                    // <Tooltip className="popup-error" id="popup-1">
                    //     {errText}
                    // </Tooltip>
                )}
            </Overlay>
        </>
    )
}
