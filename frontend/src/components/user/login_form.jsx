import {useForm, SubmitHandler} from "react-hook-form"
import {check_login, login_api} from "../backend_api/login"
import usePersistentState from "../../_helpers/UsePersistent";
import { redirect, useNavigate } from "react-router-dom";
import { useEffect, useState } from "react";
import useCustomFetch from "../../_helpers/CustomFetchHook";
export default function LoginForm({userState}) {
    const onSubmit = (data) => sendRequest(data.email,data.password);
    const {register, handleSubmit} = useForm();
    const navigate = useNavigate();
    const [username,setUsername] = userState;
    const [isLoading,data,err,sendRequest] = useCustomFetch(login_api,(data)=>{
        setUsername(data.email);
        navigate("/");
    });
    return <>
        <form onSubmit={handleSubmit(onSubmit)}>
            <input type="email" {...register("email", {required: true})}/>
            <input htmlFor="password" type="password" {...register("password",{required:true})}/>
            <input type="submit"></input>
        </form>
        { isLoading===true &&
            <div>Loading</div>
        }
        { err!==null &&
            <div>Error</div>
        }
        { data!==null &&
            <div>{JSON.stringify(data)}</div>
        }
        {/* <button onClick={onRolePress}>GetRole</button> */}
    </>
}