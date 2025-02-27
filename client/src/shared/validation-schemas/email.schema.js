import * as Yup from 'yup'

export const emailValidationSchema = Yup.object().shape({
  email: Yup.string().email('Invalid email').label('Email'),
})
