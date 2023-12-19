import { useMutation, useQuery } from "react-query"
import {useForm, SubmitHandler} from "react-hook-form"
import {check_login, login_api} from "../backend_api/login"

export default function LoginForm() {
    const {register, handleSubmit} = useForm();
    const onSubmit = (data) => login_api(data.email,data.password);
    const onRolePress = () => check_login().then((x) => console.log(x));
    return <>
        <form onSubmit={handleSubmit(onSubmit)}>
            <input type="email" {...register("email", {required: true})}/>
            <input htmlFor="password" type="password" {...register("password",{required:true})}/>
            <input type="submit"></input>
        </form>
        <button onClick={onRolePress}>GetRole</button>
    </>
}