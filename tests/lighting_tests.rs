#[cfg(test)]

mod tests {
    use rust_ray_tracer::core::color::*;
    use rust_ray_tracer::misc::utils::*;
    use rust_ray_tracer::core::sequence::Sequence;
    use rust_ray_tracer::core::vector::Vec4;
    use rust_ray_tracer::world::lighting::*;
    use rust_ray_tracer::objects::sphere::Sphere;
    use rust_ray_tracer::objects::plane::Plane;
    use rust_ray_tracer::objects::object::*;
    use rust_ray_tracer::world::scene::Scene;
    use rust_ray_tracer::materials::material::Material;
    use rust_ray_tracer::core::matrix::Matrix4x4;
    use rust_ray_tracer::ray_tracing::ray::Ray;
    use rust_ray_tracer::core::comp::Comp;
    use rust_ray_tracer::ray_tracing::intersection::Intersection;
    use rust_ray_tracer::materials::patterns::*;

    //Tests shadows when sphere does not block the light source from the point
    #[test]
    fn point_not_blocked() {
        let scene = Scene::default();
        let light = &scene.light_sources[0];
        let target = Vec4::new(0.0, 10.0, 0.0, 1.0);
        assert_eq!(in_shadow(&light.get_position(), &target, &scene), false);
    }

    //Tests shadows when sphere blocks point from light source
    #[test]
    fn shadow_blocked_sphere() {
        let scene = Scene::default();
        let light = &scene.light_sources[0];
        let target = Vec4::new(10.0, -10.0, 10.0, 1.0);
        assert_eq!(in_shadow(&light.get_position(), &target, &scene), true);
    }

    //Tests shadows when the point is in front of the light source
    #[test]
    fn point_past_light() {
        let scene = Scene::default();
        let light = &scene.light_sources[0];
        let target = Vec4::new(-20.0, 20.0, -20.0, 1.0);
        assert_eq!(in_shadow(&light.get_position(), &target, &scene), false);
    }

    //Tests creating a new AreaLight
    #[test]
    fn create_area_light() {
        let corner = Vec4::new(0.0, 0.0, 0.0, 1.0);
        let v1 = Vec4::new(2.0, 0.0, 0.0, 0.0);
        let v2 = Vec4::new(0.0, 0.0, 1.0, 0.0);
        let light = AreaLight::new(corner.clone(), v1, 4, v2.clone(), 2, WHITE);
        assert_eq!(light.corner, corner);
        assert_eq!(light.uvec, Vec4::new(0.5, 0.0, 0.0, 0.0));
        assert_eq!(light.vvec, Vec4::new(0.0, 0.0, 0.5, 0.0));
        assert_eq!(light.vsteps, 2);
        assert_eq!(light.usteps, 4);
        assert_eq!(light.samples, (8 as usize));
        assert_eq!(light.get_intensity(), &Color::new(1.0, 1.0, 1.0));
    }



    #[test]
    //Tests the Sequence struct
    fn test_sequence() {
        let list = vec![1.0, 2.0, 3.0];
        let mut sequence = Sequence::new(list);
        assert_eq!(sequence.next(), 1.0);
        assert_eq!(sequence.next(), 2.0);
        assert_eq!(sequence.next(), 3.0);
    }

    #[test]
    //Tests n1 and n2 from comps
    fn n1_n2_comps() {
        let mut material1 = Material::default();
        material1.refractive_index = 1.5;
        let a = Sphere::new(Matrix4x4::scaling(2.0, 2.0, 2.0), material1);

        let mut material2 = Material::default();
        material2.refractive_index = 2.0;
        let b = Sphere::new(Matrix4x4::translation(0.0, 0.0, -0.25), material2);

        let mut material3 = Material::default();
        material3.refractive_index = 2.5;
        let c = Sphere::new(Matrix4x4::translation(0.0, 0.0, 0.25), material3);

        let ray = Ray::new((0.0, 0.0, -4.0), (0.0, 0.0, 1.0));

        let interesections = vec![
            Intersection::new(
                2.0,
                Ray::position(&ray, 2.0),
                a.normal(&Ray::position(&ray, 2.0), None, None),
                &a,
            ),
            Intersection::new(
                2.75,
                Ray::position(&ray, 2.75),
                b.normal(&Ray::position(&ray, 2.75), None, None),
                &b,
            ),
            Intersection::new(
                3.25,
                Ray::position(&ray, 3.25),
                c.normal(&Ray::position(&ray, 3.25), None, None),
                &c,
            ),
            Intersection::new(
                4.75,
                Ray::position(&ray, 4.75),
                b.normal(&Ray::position(&ray, 4.75), None, None),
                &b,
            ),
            Intersection::new(
                5.25,
                Ray::position(&ray, 5.25),
                c.normal(&Ray::position(&ray, 5.25), None, None),
                &c,
            ),
            Intersection::new(
                6.0,
                Ray::position(&ray, 6.0),
                a.normal(&Ray::position(&ray, 6.0), None, None),
                &a,
            )
        ];

        let mut comp_list = vec![];

        for x in interesections.clone() {
            comp_list.push(Comp::compute_vars(x, &ray, &interesections));
        }

        assert_eq!(comp_list[0].n1, 1.0);
        assert_eq!(comp_list[0].n2, 1.5);
        assert_eq!(comp_list[1].n1, 1.5);
        assert_eq!(comp_list[1].n2, 2.0);
        assert_eq!(comp_list[2].n1, 2.0);
        assert_eq!(comp_list[2].n2, 2.5);
        assert_eq!(comp_list[3].n1, 2.5);
        assert_eq!(comp_list[3].n2, 2.5);
        assert_eq!(comp_list[4].n1, 2.5);
        assert_eq!(comp_list[4].n2, 1.5);
        assert_eq!(comp_list[5].n1, 1.5);
        assert_eq!(comp_list[5].n2, 1.0);
    }

    #[test]
    //Tests comp under_point
    fn under_point() {
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));

        let mut object = Sphere::glass();
        object.transform = Matrix4x4::translation(0.0, 0.0, 1.0);

        let i = Intersection::new(
            5.0,
            Ray::position(&ray, 5.0),
            object.normal(&Ray::position(&ray, 5.0), None, None),
            &object,
        );

        let comps = Comp::compute_vars(i.clone(), &ray, &vec![i]);
        if comps.under_point.2 > EPSILON_BUMP / 2.0 {
            panic!("Under point test failed");
        }

        if comps.point.2 > comps.under_point.2 {
            panic!("Under point test failed");
        }
    }

    #[test]
    //Test refracted color of an opaque surface
    fn refraction_on_opaque() {
        let scene = Scene::default();
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let intersections = vec![
            Intersection::new(
                4.0,
                Ray::position(&ray, 4.0),
                scene.objects[0].normal(&Ray::position(&ray, 4.0), None, None),
                &*scene.objects[0],
            ),
            Intersection::new(
                6.0,
                Ray::position(&ray, 6.0),
                scene.objects[0].normal(&Ray::position(&ray, 6.0), None, None),
                &*scene.objects[0],
            )
        ];
        let comps = Comp::compute_vars(intersections[0].clone(), &ray, &intersections);
        let color = refracted_color(&scene, &comps, 5);
        assert_eq!(color, BLACK);
    }

    #[test]
    //Test refracted color over recursion limit
    fn refraction_under_recursion_limit() {
        let scene = Scene {
            light_sources: vec![Box::new(PointLight::new(
                Color::new(1.0, 1.0, 1.0),
                Vec4::new(-10.0, 10.0, -10.0, 1.0),
            ))],
            objects: vec![
                Box::new(Sphere::new(
                    Matrix4x4::identity(),
                    Material::new(
                        Color::new(0.8, 1.0, 0.6),
                        0.1,
                        0.7,
                        0.2,
                        200.0,
                        0.0,
                        1.0,
                        1.5,
                        0.0,
                        true,
                        None,
                    ),
                )),
                Box::new(Sphere::new(
                    Matrix4x4::scaling(0.5, 0.5, 0.5),
                    Material::default(),
                )),
            ],
        };

        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));

        let intersections = vec![
            Intersection::new(
                4.0,
                Ray::position(&ray, 4.0),
                scene.objects[0].normal(&Ray::position(&ray, 4.0), None, None),
                &*scene.objects[0],
            ),
            Intersection::new(
                6.0,
                Ray::position(&ray, 6.0),
                scene.objects[0].normal(&Ray::position(&ray, 6.0), None, None),
                &*scene.objects[0],
            )
        ];

        let comps = Comp::compute_vars(intersections[0].clone(), &ray, &intersections);
        let color = refracted_color(&scene, &comps, 0);
        assert_eq!(color, BLACK);
    }

    #[test]
    //Test refracted color with total internal refraction
    //Total internal refraction occurs when a light ray enters a new medium with a lower refractive index at an acute angle
    fn refraction_total_internal_refraction() {
        let scene = Scene {
            light_sources: vec![Box::new(PointLight::new(
                Color::new(1.0, 1.0, 1.0),
                Vec4::new(-10.0, 10.0, -10.0, 1.0),
            ))],
            objects: vec![
                Box::new(Sphere::new(
                    Matrix4x4::identity(),
                    Material::new(
                        Color::new(0.8, 1.0, 0.6),
                        0.1,
                        0.7,
                        0.2,
                        200.0,
                        0.0,
                        1.0,
                        1.5,
                        0.0,
                        true,
                        None,
                    ),
                )),
                Box::new(Sphere::new(
                    Matrix4x4::scaling(0.5, 0.5, 0.5),
                    Material::default(),
                )),
            ],
        };

        let ray = Ray::new((0.0, 0.0, (2.0 as f32).sqrt() / 2.0), (0.0, 1.0, 0.0));

        let intersections = vec![
            Intersection::new(
                -(2.0 as f32).sqrt() / 2.0,
                Ray::position(&ray, -(2.0 as f32).sqrt() / 2.0),
                scene.objects[0].normal(&Ray::position(&ray, -(2.0 as f32).sqrt() / 2.0), None, None),
                &*scene.objects[0],
            ),
            Intersection::new(
                (2.0 as f32).sqrt() / 2.0,
                Ray::position(&ray, (2.0 as f32).sqrt() / 2.0),
                scene.objects[0].normal(&Ray::position(&ray, (2.0 as f32).sqrt() / 2.0), None, None),
                &*scene.objects[0],
            )
        ];

        let comps = Comp::compute_vars(intersections[1].clone(), &ray, &intersections);
        let color = refracted_color(&scene, &comps, 5);
        assert_eq!(color, BLACK);
    }

    #[test]
    //Test refracted color calculation
    fn refracted_color_test() {
        let mut material = Material::default();
        material.transparency = 1.0;
        material.refractive_index = 1.5;
        let scene = Scene {
            light_sources: vec![Box::new(PointLight::new(
                Color::new(1.0, 1.0, 1.0),
                Vec4::new(-10.0, 10.0, -10.0, 1.0),
            ))],
            objects: vec![
                Box::new(Sphere::new(
                    Matrix4x4::identity(),
                    Material::new(
                        Color::new(0.8, 1.0, 0.6),
                        1.0,
                        0.7,
                        0.2,
                        200.0,
                        0.0,
                        0.0,
                        1.0,
                        0.0,
                        true,
                        Some(Box::new(TestPattern::new(Matrix4x4::identity()))),
                    ),
                )),
                Box::new(Sphere::new(
                    Matrix4x4::scaling(0.5, 0.5, 0.5),
                    material,
                )),
            ],
        };

        let ray = Ray::new((0.0, 0.0, 0.1), (0.0, 1.0, 0.0));

        let intersections = vec![
            Intersection::new(
                -0.9899,
                Ray::position(&ray, -0.9899),
                scene.objects[0].normal(&Ray::position(&ray, -0.9899), None, None),
                &*scene.objects[0],
            ),
            Intersection::new(
                -0.4899,
                Ray::position(&ray, -0.4899),
                scene.objects[1].normal(&Ray::position(&ray, -0.4899), None, None),
                &*scene.objects[1],
            ),
            Intersection::new(
                0.4899,
                Ray::position(&ray, 0.4899),
                scene.objects[1].normal(&Ray::position(&ray, 0.4899), None, None),
                &*scene.objects[1],
            ),
            Intersection::new(
                0.9899,
                Ray::position(&ray, 0.9899),
                scene.objects[0].normal(&Ray::position(&ray, 0.9899), None, None),
                &*scene.objects[0],
            )
        ];

        let comps = Comp::compute_vars(intersections[2].clone(), &ray, &intersections);
        let color = refracted_color(&scene, &comps, 5);
        assert_eq!(color.round(), Color(0.0, 0.9988, 0.0472).round());
    }

    #[test]
    //Tests color with refraction
    fn color_with_refraction() {
        let mut material = Material::default();
        material.transparency = 1.0;
        material.refractive_index = 1.5;

        let mut floor_material = Material::default();
        floor_material.transparency = 0.5;
        floor_material.refractive_index = 1.5;

        let floor = Plane::new(Matrix4x4::translation(0.0, -1.0, 0.0), floor_material);

        let mut ball_material = Material::default();
        ball_material.color = Color::new(1.0, 0.0, 0.0);
        ball_material.ambient = 0.5;

        let ball = Sphere::new(Matrix4x4::translation(0.0, -3.5, -0.5), ball_material);
       
        let scene = Scene {
            light_sources: vec![Box::new(PointLight::new(
                Color::new(1.0, 1.0, 1.0),
                Vec4::new(-10.0, 10.0, -10.0, 1.0),
            ))],
            objects: vec![
                Box::new(Sphere::new(
                    Matrix4x4::identity(),
                    Material::new(
                        Color::new(0.8, 1.0, 0.6),
                        0.1,
                        0.7,
                        0.2,
                        200.0,
                        0.0,
                        0.0,
                        1.0,
                        0.0,
                        true,
                        None,
                    ),
                )),
                Box::new(Sphere::new(
                    Matrix4x4::scaling(0.5, 0.5, 0.5),
                    Material::default(),
                )),
                Box::new(floor),
                Box::new(ball),
            ],
        };

        let ray = Ray::new((0.0, 0.0, -3.0), (0.0, -((2.0 as f32).sqrt() / 2.0), (2.0 as f32).sqrt() / 2.0));

        let intersections = vec![
            Intersection::new(
                (2.0 as f32).sqrt(),
                Ray::position(&ray, (2.0 as f32).sqrt()),
                scene.objects[2].normal(&Ray::position(&ray, (2.0 as f32).sqrt()), None, None),
                &*scene.objects[2],
            ),
        ];

        let comps = Comp::compute_vars(intersections[0].clone(), &ray, &intersections);
        let color = Scene::scene_lighting(&scene, &comps, 5);
        assert_eq!(color.round(), Color(0.93642, 0.68642, 0.68642).round());
    }
}
