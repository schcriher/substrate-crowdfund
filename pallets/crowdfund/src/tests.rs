use crate::{mock::*, tipos::NombreProyecto, Error, Proyectos};
use codec::Encode;
use frame_support::{assert_noop, assert_ok};

#[test]
fn crear_proyecto_funciona() {
	new_test_ext().execute_with(|| {
		let nombre_corto = "A".encode();
		let nombre_largo = "Supercalifragilisticoexpialidoso".encode();
		let nombre_proyecto = "Proyecto #1".encode();
		let nombre_acotado: NombreProyecto<Test> = nombre_proyecto.clone().try_into().unwrap();
		assert_eq!(Proyectos::<Test>::contains_key(nombre_acotado.clone()), false);
		assert_noop!(
			Crowdfund::crear_proyecto(RuntimeOrigin::signed(1), nombre_corto),
			Error::<Test>::NombreMuyCorto
		);
		assert_noop!(
			Crowdfund::crear_proyecto(RuntimeOrigin::signed(1), nombre_largo),
			Error::<Test>::NombreMuyLargo
		);
		assert_ok!(Crowdfund::crear_proyecto(RuntimeOrigin::signed(1), nombre_proyecto));
		assert_eq!(Proyectos::<Test>::contains_key(nombre_acotado.clone()), true);
		assert_eq!(Proyectos::<Test>::get(nombre_acotado), 0);
	});
}

#[test]
fn apoyar_proyecto_funciona() {
	new_test_ext().execute_with(|| {
		let nombre_proyecto = "Mi proyecto".encode();
		let nombre_acotado: NombreProyecto<Test> = nombre_proyecto.clone().try_into().unwrap();
		assert_ok!(Crowdfund::crear_proyecto(RuntimeOrigin::signed(1), nombre_proyecto.clone()));
		assert_ok!(Crowdfund::apoyar_proyecto(RuntimeOrigin::signed(2), nombre_proyecto, 500));
		assert_eq!(Proyectos::<Test>::get(nombre_acotado), 500);
	});
}

#[test]
fn evento_proyecto_ya_existe_funciona() {
	new_test_ext().execute_with(|| {
		let nombre_a = "Mi proyecto #1".encode();
		let nombre_b = "Mi proyecto #1".encode();

		assert_ok!(Crowdfund::crear_proyecto(RuntimeOrigin::signed(1), nombre_a.clone()));

		assert_noop!(
			Crowdfund::crear_proyecto(RuntimeOrigin::signed(1), nombre_a.clone()), // misma variable
			Error::<Test>::ProyectoYaExiste
		);
		assert_noop!(
			Crowdfund::crear_proyecto(RuntimeOrigin::signed(1), nombre_b.clone()), // otra variable
			Error::<Test>::ProyectoYaExiste
		);
		assert_noop!(
			Crowdfund::crear_proyecto(RuntimeOrigin::signed(2), nombre_b.clone()), // otra variable
			Error::<Test>::ProyectoYaExiste
		);
	});
}

#[test]
fn evento_cantidad_debe_ser_mayor_a_cero_funciona() {
	new_test_ext().execute_with(|| {
		let nombre = "Mi proyecto".encode();
		assert_ok!(Crowdfund::crear_proyecto(RuntimeOrigin::signed(1), nombre.clone()));

		assert_noop!(
			Crowdfund::apoyar_proyecto(RuntimeOrigin::signed(1), nombre.clone(), 0),
			Error::<Test>::CantidadDebeSerMayorACero
		);
		assert_noop!(
			Crowdfund::apoyar_proyecto(RuntimeOrigin::signed(2), nombre.clone(), 0),
			Error::<Test>::CantidadDebeSerMayorACero
		);
	});
}

