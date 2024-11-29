import * as Yup from 'yup'

export const loginValidationSchema = Yup.object().shape({
  email: Yup.string()
    .email('Invalid email')
    .required('Required')
    .label('Email'),
  password: Yup.string().min(8).required('Required').label('Password'),
})
