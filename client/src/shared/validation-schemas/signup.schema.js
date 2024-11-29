import * as Yup from 'yup'

export const signupValidationSchema = Yup.object().shape({
  email: Yup.string()
    .email('Invalid email')
    .required('Required')
    .label('Email'),
  password: Yup.string().min(8).required('Required').label('Password'),
  confirmPassword: Yup.string()
    .oneOf([Yup.ref('password')], 'Passwords do not match')
    .required('Required')
    .label('Confirm password'),
})
