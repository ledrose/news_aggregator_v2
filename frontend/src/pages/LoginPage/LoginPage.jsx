import { Col,Row,Container } from "react-bootstrap"
import LoginForm from "../../components/user/login_form"
import "./LoginPage.css";
import ScrollLayout from "../../Layouts/ScrollLayout/ScrollLayout";
import StaticLayout from "../../Layouts/StaticLayout/StaticLayout";
export default function LoginPage() {
    return <StaticLayout>
        <LoginForm/>
    </StaticLayout>
}


